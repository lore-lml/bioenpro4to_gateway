import requests as req


def main():
    url = "http://localhost:8080"
    headers = {
        "id": "m1111",
        "did": "did:iota:GLBdxx1ZZxwHva9hCL6bRj4inYQJbJJBu3jKnCYJhQFU"
    }
    res = req.get(f"{url}/id-manager/channel-credential", headers=headers)
    cred = res.json()
    print(cred)

    res = req.get(f"{url}/id-manager/is-credential-valid", json=cred)
    print(res.json())

if __name__ == '__main__':
    main()
