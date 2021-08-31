import requests as req
from time import time
import json as js

def main():
    url = "http://localhost:8000"
    json = {
        "id": "aa000aa",
        "did": "did:iota:test:HRyXLr22JbT4VYczsFRB3p7T5xnHDReHU78d4Ns7RqAa",
        "psw": "ciao"
    }
    res = req.post(f"{url}/id-manager/authenticate", json=json)
    cred = res.json()
    print(cred)

    res = req.get(f"{url}/id-manager/is-credential-valid", json=cred)
    print(res.text)

    json = {
        "Cred": js.dumps(cred),
        "Channel-psw": "psw"
    }
    res = req.post(f"{url}/channel-manager/daily-channel", headers=json, json={"day_timestamp": int(time())})
    print(res.text)

    res = req.get(f"{url}/channel-manager/daily-channel/31-08-2021", headers=json)
    print(res)
    print(res.text)

if __name__ == '__main__':
    main()
