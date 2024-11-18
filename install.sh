/usr/bin/python3 -m venv venv
source ./venv/bin/activate
pip3 install -r ./requirements.txt

sudo apt install curl
curl -fsSL https://get.docker.com -o get-docker.sh
sudo sh get-docker.sh
