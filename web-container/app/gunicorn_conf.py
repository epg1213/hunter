# Gunicorn config variables
loglevel = "info"
errorlog = "/var/log/error.log"  # stderr
accesslog = "/var/log/access.log"  # stdout
worker_tmp_dir = "/dev/shm"
graceful_timeout = 120
timeout = 120
keepalive = 5
threads = 3
