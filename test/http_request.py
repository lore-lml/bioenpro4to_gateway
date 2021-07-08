import requests as req
from time import time

def main():
    url = "http://localhost:8080"
    headers = {
        "id": "m1111",
        "did": "did:iota:GLBdxx1ZZxwHva9hCL6bRj4inYQJbJJBu3jKnCYJhQFU"
    }
    res = req.get(f"{url}/id-manager/channel-credential", headers=headers)
    cred = res.json()
    print(cred)

    # res = req.get(f"{url}/id-manager/is-credential-valid", json=cred)
    # print(res.json())

    json = {
        "cred": cred,
        "day_timestamp": int(time()),
        "psw": "psw"
    }
    res = req.post(f"{url}/id-manager/daily-channel", json=json)
    print(res.text)

if __name__ == '__main__':
    main()
