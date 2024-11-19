echo "Checking for docker installation..."
if ! docker --version; then
	echo "Docker doesn't seem to be installed"
    sudo apt install curl
    curl -fsSL https://get.docker.com -o get-docker.sh
    sudo sh get-docker.sh
fi

echo "Setting up Database containter..."
DBPASS=$(tr -dc 'A-Za-z0-9!#$%&'\''()*+,-./:;<=>?@[\]^_`{|}~' </dev/urandom | head -c 16)
echo $DBPASS > .dbpass
sudo docker stop hunter-mysql
sudo docker rm hunter-mysql
sudo docker run --name hunter-mysql -p 42601:3306 -e MYSQL_ROOT_PASSWORD=$DBPASS -e MYSQL_DATABASE=hunting -d mysql:latest

echo "Preparing management script \"hunter\"..."
echo "alias hunter=\"$(pwd)/hunter\"" >> ~/.bash_aliases

echo "Creating python virtual environment..."
/usr/bin/python3 -m venv venv
source ./venv/bin/activate

echo "Installing required python packages..."
pip3 install -r ./requirements.txt

echo "Setting up Database..."
python3 models/database.py
