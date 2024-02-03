import consul


def register_consul(service_name, ip, port):
    c = consul.Consul()
    service_id = f"{service_name}-{port}"

    result = c.agent.service.register(
        name=service_name,
        service_id=service_id,
        address=ip,
        port=port,
        check=consul.Check().tcp(ip, port, "10s", "30s", "30s")
    )
    if result:
        print("Service registration successful")
    else:
        print("Service registration failed")


def unregister_consul(service_id):
    c = consul.Consul()
    result = c.agent.service.deregister(service_id=service_id)
    if result:
        print(f"Service deregister {service_id} successful")
    else:
        print(f"Service deregister {service_id} failed")
