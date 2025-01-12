# Gunicorn config variables
loglevel = "info"
errorlog = "/dev/null"  # stderr
accesslog = "/dev/null"  # stdout
worker_tmp_dir = "/dev/shm"
graceful_timeout = 120
timeout = 120
keepalive = 5
threads = 3
