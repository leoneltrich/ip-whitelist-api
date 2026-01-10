import argparse
import os

SERVICES = {
    "auth": {
        "upstream_name": "auth_service",
        "port": 3000,
        "routes": "\n        # --- Auth Service Routes ---        \n        location /login {        \n            proxy_pass http://auth_service;        \n            proxy_set_header Host $host;        \n            proxy_set_header X-Real-IP $remote_addr;        \n        }        \n        \n        location /token {        \n            proxy_pass http://auth_service;        \n            proxy_set_header Host $host;        \n            proxy_set_header X-Real-IP $remote_addr;        \n        }        \n\n        location /admin/users {        \n            proxy_pass http://auth_service;        \n            proxy_set_header Host $host;        \n            proxy_set_header X-Real-IP $remote_addr;        \n        }        \n        \n        location /users/profile {        \n            proxy_pass http://auth_service;        \n            proxy_set_header Host $host;        \n            proxy_set_header X-Real-IP $remote_addr;        \n        }        \n\n        location /auth/health {        \n            proxy_pass http://auth_service/health;        \n            proxy_set_header Host $host;        \n            proxy_set_header X-Real-IP $remote_addr;        \n        }        \n\n        location /auth/swagger-ui/ {        \n            rewrite ^/auth/swagger-ui/(.*) /swagger-ui/$1 break;        \n            proxy_pass http://auth_service;        \n        }        \n        \n        location /auth/api-docs/ {        \n             rewrite ^/auth/api-docs/(.*) /api-docs/$1 break;        \n             proxy_pass http://auth_service;        \n        }        "
    },
    "firewall": {
        "upstream_name": "firewall_service",
        "port": 3001,
        "routes": "\n        # --- Firewall Service Routes ---        \n        location /admin/servers {        \n            proxy_pass http://firewall_service;        \n            proxy_set_header Host $host;        \n            proxy_set_header X-Real-IP $remote_addr;        \n        }        \n        \n        location /users/access {        \n            proxy_pass http://firewall_service;        \n            proxy_set_header Host $host;        \n            proxy_set_header X-Real-IP $remote_addr;        \n        }        \n        \n        location /users/servers {        \n            proxy_pass http://firewall_service;        \n            proxy_set_header Host $host;        \n            proxy_set_header X-Real-IP $remote_addr;        \n        }        \n\n        location /health {        \n            proxy_pass http://firewall_service;        \n            proxy_set_header Host $host;        \n            proxy_set_header X-Real-IP $remote_addr;        \n        }        \n\n        location /firewall/swagger-ui/ {        \n            rewrite ^/firewall/swagger-ui/(.*) /swagger-ui/$1 break;        \n            proxy_pass http://firewall_service;        \n        }        \n        \n        location /firewall/api-docs/ {        \n             rewrite ^/firewall/api-docs/(.*) /api-docs/$1 break;        \n             proxy_pass http://firewall_service;        \n        }        "
    },
    "notes": {
        "upstream_name": "notes_service",
        "port": 3002,
        "routes": "\n        # --- Notes Service Routes ---        \n        location /notes {        \n            proxy_pass http://notes_service;        \n            proxy_set_header Host $host;        \n            proxy_set_header X-Real-IP $remote_addr;        \n        }        "
    }
}

NGINX_TEMPLATE = """events {{    worker_connections 1024}}http {{{upstreams}    server {{        listen 80;{routes}    }}}}\n"""

def generate_nginx_conf(enabled_services, output_path):
    upstreams = ""
    routes = ""
    
    for service_name in enabled_services:
        if service_name in SERVICES:
            s = SERVICES[service_name]
            upstreams += f"    upstream {s['upstream_name']} {{\n        server {s['upstream_name']}:{s['port']};\n    }}\n\n"
            routes += s['routes']
        else:
            print(f"⚠️  Unknown service: {service_name}")

    conf_content = NGINX_TEMPLATE.format(upstreams=upstreams, routes=routes)
    
    with open(output_path, "w") as f:
        f.write(conf_content)
    
    print(f"✅ nginx.conf generated successfully at: {output_path}")
    print(f"📦 Services included: {', '.join(enabled_services)}")

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Generate Nginx configuration for Access Proxy services.")
    parser.add_argument("services", nargs="+", help="Services to enable (auth, firewall, notes)")
    parser.add_argument("--output", "-o", default="nginx.conf", help="Output path for nginx.conf (default: nginx.conf)")
    
    args = parser.parse_args()
    generate_nginx_conf(args.services, args.output)
