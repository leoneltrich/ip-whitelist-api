import json
import os
import re
import subprocess

def get_input(prompt, default=None):
    val = input(f"{prompt} [{default}]: " if default else f"{prompt}: ").strip()
    return val if val else default

def load_spec_from_file(path):
    if not os.path.exists(path):
        raise FileNotFoundError(f"Spec file not found: {path}")
    with open(path, 'r') as f:
        return json.load(f)

def run_docker_command(cmd):
    """Run a shell command and return output."""
    try:
        # shell=True is acceptable here as this is a local dev tool
        result = subprocess.run(cmd, shell=True, check=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        return result.stdout.decode('utf-8').strip()
    except subprocess.CalledProcessError:
        return None

def fetch_spec_via_sidecar(container_id, port, spec_path, protocol="http"):
    """
    Spins up a temporary curl container attached to the target's network
    to fetch the spec from localhost.
    """
    print(f"   ⏳ Fetching spec from {container_id[:12]} via sidecar...")
    # Use curlimages/curl to hit localhost inside the target's namespace
    # --rm: clean up after itself
    # --network container:ID: share the network stack (localhost access)
    cmd = (
        f"docker run --rm --network container:{container_id} "
        f"curlimages/curl -s -m 5 {protocol}://localhost:{port}{spec_path}"
    )
    json_str = run_docker_command(cmd)
    
    if not json_str:
        return None
        
    try:
        return json.loads(json_str)
    except json.JSONDecodeError:
        print(f"   ❌ Failed to parse JSON from container. Response: {json_str[:50]}...")
        return None

def discover_docker_services(label_prefix="proxy"):
    """
    Scans for running containers with label '{label_prefix}.enable=true'.
    """
    print("\n🐳 Docker Auto-Discovery")
    print(f"   Scanning for labels starting with '{label_prefix}.'...")
    print("-----------------------")
    
    # Check if docker is available
    if not run_docker_command("docker --version"):
        print("❌ Docker CLI not found or not running.")
        return []

    # List containers with formatting to extract ID, Names, and Labels
    # Format: ID|Names|Labels
    # Labels are comma-separated list of key=value
    raw_output = run_docker_command('docker ps --format "{{.ID}}|{{.Names}}|{{.Labels}}"')
    if not raw_output:
        print("⚠️  No running containers found.")
        return []

    discovered = []
    
    for line in raw_output.split('\n'):
        parts = line.split('|')
        if len(parts) < 3: continue
        
        c_id, c_name, labels_raw = parts
        
        # Parse labels into a dict
        labels = {}
        if labels_raw:
            for l in labels_raw.split(','):
                if '=' in l:
                    k, v = l.split('=', 1)
                    labels[k.strip()] = v.strip()
        
        # STRICT FILTER: Only process containers with explicit enable label
        if labels.get(f'{label_prefix}.enable') != 'true':
            continue

        print(f"🔎 Found candidate: {c_name}")
        
        # Extract state from labels
        port = labels.get(f'{label_prefix}.port', '3000')
        protocol = labels.get(f'{label_prefix}.protocol', 'http')
        prefix = labels.get(f'{label_prefix}.prefix', '')
        spec_url = labels.get(f'{label_prefix}.spec_path', '/api-docs/openapi.json')
        
        # Fetch Spec
        spec = fetch_spec_via_sidecar(c_id, port, spec_url, protocol)
        if not spec:
            print(f"   ⚠️  Skipping {c_name}: Could not retrieve OpenAPI spec from {spec_url}.")
            continue
            
        # Parse Paths
        raw_paths = spec.get('paths', {})
        resolved_paths = []
        for path in raw_paths.keys():
            full_path = path
            if prefix:
                p = prefix.strip('/')
                sub = path.lstrip('/')
                full_path = f"/{p}/{sub}"
            resolved_paths.append(full_path)

        # Service Name Logic
        service_title = spec.get('info', {}).get('title', c_name)
        service_name = service_title.replace(" ", "_").lower()
        
        # Determine Hostname: Prefer Docker Compose Service Name -> Container Name
        # This ensures stable networking (e.g. 'auth_service' instead of 'project-auth_service-1')
        hostname = labels.get('com.docker.compose.service', c_name)

        discovered.append({
            "name": hostname,           # Network hostname (Docker container name)
            "upstream_name": service_name, # Nginx upstream block name
            "ip": hostname,             # For Docker, Hostname == IP
            "port": port,
            "protocol": protocol,
            "prefix": prefix,
            "paths": resolved_paths
        })
        print(f"   ✅ Added {service_name} ({len(resolved_paths)} paths)")

    return discovered

def generate_nginx_conf(services, final_routing, output_path):
    """
    final_routing: Dict[full_path, service_index]
    """
    upstreams = []
    # Group paths by service index for generation
    service_locations = {idx: [] for idx in range(len(services))}
    
    for full_path, s_idx in final_routing.items():
        s = services[s_idx]
        rewrite_rule = ""
        if s['prefix']:
            rewrite_rule = f"\n                rewrite ^{s['prefix']}/(.*) /$1 break;"
        
        # Handle path parameters (e.g. {id}) by converting to Nginx Regex
        if '{' in full_path and '}' in full_path:
            segments = full_path.split('/')
            regex_segments = []
            for seg in segments:
                if seg.startswith('{') and seg.endswith('}'):
                    # Convert {param} to capture group excluding slashes
                    regex_segments.append('([^/]+)')
                else:
                    # Escape literal parts of the path
                    regex_segments.append(re.escape(seg))
            
            regex_path = '/'.join(regex_segments)
            # Use regex location modifier '~'
            location_block = f"location ~ ^{regex_path}$"
        else:
            location_block = f"location {full_path}"
        
        # Use upstream_name for the internal nginx reference
        upstream_ref = s.get('upstream_name', s['name'])
        
        loc = f"""        {location_block} {{
                proxy_pass {s['protocol']}://{upstream_ref};{rewrite_rule}
                proxy_set_header Host $host;
                proxy_set_header X-Real-IP $remote_addr;
                proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
            }}"""
        service_locations[s_idx].append(loc)

    # Generate Upstreams
    # Ensure unique names
    unique_names = set()
    for s in services:
        name = s.get('upstream_name', s['name'])
        # Handle duplicates if any
        base = name
        ctr = 1
        while name in unique_names:
            name = f"{base}_{ctr}"
            ctr += 1
        s['upstream_name'] = name # Update service object with final name
        unique_names.add(name)
        
        upstreams.append(f"    upstream {name} {{\n        server {s['ip']}:{s['port']};\n    }}")

    all_locations = []
    for idx in range(len(services)):
        all_locations.extend(service_locations[idx])

    conf_content = f"""events {{ 
    worker_connections 1024;
}}

http {{ 
    set_real_ip_from 10.0.0.0/8;
    set_real_ip_from 172.16.0.0/12;
    set_real_ip_from 127.0.0.1;
    real_ip_header X-Forwarded-For;
    real_ip_recursive on;

{chr(10).join(upstreams)}

    server {{ 
        listen 80;

{chr(10).join(all_locations)}
    }}
}}
"""
    with open(output_path, "w") as f:
        f.write(conf_content)

def manual_input_phase():
    services = []
    while True:
        spec_path = get_input("Path to OpenAPI JSON spec")
        try:
            spec = load_spec_from_file(spec_path)
        except Exception as e:
            print(f"❌ Error loading spec: {e}")
            continue
            
        ip = get_input("Destination IP/Hostname", "127.0.0.1")
        port = get_input("Destination Port", "3000")
        protocol = get_input("Protocol (http/https)", "http").lower()
        prefix = get_input("Path Prefix (optional)", "")
        
        service_info = spec.get('info', {})
        service_title = service_info.get('title', f"service_{len(services)}")
        service_name = service_title.replace(" ", "_").lower()

        raw_paths = spec.get('paths', {})
        resolved_paths = []
        for path in raw_paths.keys():
            full_path = path
            if prefix:
                p = prefix.strip('/')
                sub = path.lstrip('/')
                full_path = f"/{p}/{sub}"
            resolved_paths.append(full_path)

        services.append({
            "name": service_name,
            "upstream_name": service_name,
            "ip": ip,
            "port": port,
            "protocol": protocol,
            "prefix": prefix,
            "paths": resolved_paths
        })
        
        more = get_input("Add another service? (y/n)", "n").lower()
        if more != 'y':
            break
    return services

def main():
    print("🚀 Nginx Dynamic Configuration Generator")
    print("----------------------------------------")
    print("1. Manual Input (Local Files)")
    print("2. Docker Auto-Discovery (Labels)")
    
    choice = get_input("Select mode", "1")
    
    services = []
    if choice == '2':
        label_prefix = get_input("Docker Label Prefix", "proxy")
        services = discover_docker_services(label_prefix)
        if not services:
            print("⚠️  No services found via Docker. Falling back to manual.")
            services = manual_input_phase()
    else:
        services = manual_input_phase()

    if not services:
        print("❌ No services configured. Exiting.")
        return

    # --- Host Mode Selection ---
    is_host_mode = get_input("\nWill Nginx run in 'network_mode: host'?", "n").lower() == 'y'

    if is_host_mode:
        print("\n" + "!" * 65)
        print("⚠️  IMPORTANT: NGINX HOST MODE DETECTED")
        print("-" * 65)
        print(" All upstreams are being forced to 127.0.0.1.")
        print(" You MUST ensure your docker-compose.yml has explicit port mappings")
        print(" for every service (e.g., 127.0.0.1:3000:3000) so Nginx can reach")
        print(" them from the host network.")
        print("!" * 65 + "\n")

        for s in services:
            s['ip'] = '127.0.0.1'

    # 2. Conflict Analysis Phase
    path_providers = {} # full_path -> [service_index, ...]
    for idx, s in enumerate(services):
        for p in s['paths']:
            if p not in path_providers:
                path_providers[p] = []
            path_providers[p].append(idx)

    # 3. Resolution Phase
    final_routing = {} # full_path -> service_index
    
    print("\n🔍 Analyzing routes and resolving conflicts...")
    
    for path, providers in path_providers.items():
        if len(providers) == 1:
            final_routing[path] = providers[0]
        else:
            print(f"\n⚠️  Conflict detected for path: {path}")
            print("Which service should handle this endpoint?")
            for i, s_idx in enumerate(providers):
                s = services[s_idx]
                print(f"  {i+1}. {s['upstream_name']} ({s['ip']}:{s['port']})")
            
            while True:
                choice = get_input(f"Select 1-{len(providers)}", "1")
                try:
                    choice_idx = int(choice) - 1
                    if 0 <= choice_idx < len(providers):
                        final_routing[path] = providers[choice_idx]
                        break
                    else:
                        print(f"Please enter a number between 1 and {len(providers)}.")
                except ValueError:
                    print("Invalid input. Please enter a number.")

    # 4. Generation Phase
    output_file = get_input("\nOutput filename", "nginx.conf")
    print(f"🔨 Generating configuration for {len(services)} services...")
    generate_nginx_conf(services, final_routing, output_file)
    print(f"✅ Done! Configuration saved to {output_file}")

if __name__ == "__main__":
    main()
