import requests as req
from time import time

def main():
    url = "http://localhost:8000"
    headers = {
        "id": "D111",
        "did": "did:iota:test:BG6DuW2ESTyvLR2CJA4GJAT53NfMJohZYjmfWRiGySeg",
        "category": "biocells"
    }
    res = req.get(f"{url}/id-manager/actor-nonce", headers=headers)
    nonce = res.text
    print(nonce)

    # res = req.get(f"{url}/id-manager/is-credential-valid", json=cred)
    # print(res.json())
    #
    # json = {
    #     "cred": cred,
    #     "day_timestamp": int(time()),
    #     "psw": "psw"
    # }
    # res = req.get(f"{url}/channel-manager/daily-channel", json=json)
    # print(res.text)

if __name__ == '__main__':
    main()
