. ./hunter.conf
cd sql-container
sudo docker stop hunter-sql
sudo docker remove hunter-sql
sudo docker image rm hunter-sql-img
sudo docker build -t hunter-sql-img .
sudo docker run --name hunter-sql -d -p $SQLPORT:3306 -e MYSQL_ROOT_PASSWORD=$SQLPASS -e MYSQL_DATABASE=hunting hunter-sql-img
cd ..
cd web-container
sudo docker stop hunter-web
sudo docker remove hunter-web
sudo docker image rm hunter-web-img
sudo docker build -t hunter-web-img .
sudo docker run --name hunter-web -d -p $WEBPORT:80 -e MYSQL_PORT=$SQLPORT -e MYSQL_ROOT_PASSWORD=$SQLPASS hunter-web-img
cd ..
sudo docker start hunter-sql
sudo docker start hunter-web
