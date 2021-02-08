import requests


def url(endpoint):
    return f"http://127.0.0.1:8000/api/competencies/{endpoint}"


def components_url(endpoint):
    return f"http://127.0.0.1:8000/api/components/{endpoint}"


def domain_url(endpoint):
    return f"http://127.0.0.1:8000/api/domains/{endpoint}"


def test_competencies_by_component_id():
    # Get the first domain
    response = requests.get(domain_url("all"))
    domain = response.json()["domains"][0]

    # Get the first component
    response = requests.get(components_url(f"by_domain_id/{domain['id']}"))
    component = response.json()["components"][0]

    # Get competencies
    response = requests.get(url(f"by_component_id/{component['id']}"))
    assert response.status_code == 200
    j = response.json()

    # Check
    assert j["domain"] == domain
    assert j["component"] == component
    for competency in j["competencies"]:
        assert competency["component_id"] == component["id"]
