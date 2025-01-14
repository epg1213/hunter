echo "Loading configuration file..."
. ./hunter.conf
echo "Checking for docker installation..."
if ! docker --version; then
	echo "Docker doesn't seem to be installed"
    sudo apt install curl
    curl -fsSL https://get.docker.com -o get-docker.sh
    sudo sh get-docker.sh
fi
echo "Setting up Database container..."
cd sql-container
sudo docker stop hunter-sql
sudo docker remove hunter-sql
sudo docker image rm hunter-sql-img
sudo docker build -t hunter-sql-img .
sudo docker run --name hunter-sql -d -p $SQLPORT:3306 -e MYSQL_ROOT_PASSWORD=$SQLPASS -e MYSQL_DATABASE=hunting hunter-sql-img
sudo docker exec -d hunter-sql mysql -u root -p$SQLPASS hunting < ./db_setup/page.sql
sudo docker exec -d hunter-sql mysql -u root -p$SQLPASS hunting < ./db_setup/parameter.sql
sudo docker exec -d hunter-sql mysql -u root -p$SQLPASS hunting < ./db_setup/project.sql
sudo docker exec -d hunter-sql mysql -u root -p$SQLPASS hunting < ./db_setup/website.sql
cd ..
echo "Setting up Web container..."
cd web-container
sudo docker stop hunter-web
sudo docker remove hunter-web
sudo docker image rm hunter-web-img
sudo docker build -t hunter-web-img .
sudo docker run --name hunter-web -d -p $WEBPORT:80 -e MYSQL_PORT=$SQLPORT -e MYSQL_ROOT_PASSWORD=$SQLPASS hunter-web-img
