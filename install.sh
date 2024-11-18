/usr/bin/python3 -m venv venv
source ./venv/bin/activate
pip3 install -r ./requirements.txt

if ! docker --version; then
	echo "Docker doesn't seem to be installed"
    sudo apt install curl
    curl -fsSL https://get.docker.com -o get-docker.sh
    sudo sh get-docker.sh
fi
