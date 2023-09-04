GR_JUPYTER_NOTEBOOK_IP=44.198.53.192
GR_DEV_PEM_KEY=~/.ssh/gr-dev-instances-1.pem
if [[ "$1" == "ssh" ]]; then
    ssh ubuntu@${GR_JUPYTER_NOTEBOOK_IP} -i ${GR_DEV_PEM_KEY}
elif [[ "$1" == "rsync" ]]; then
    rsync -e "ssh -i ${GR_DEV_PEM_KEY}" $2 ubuntu@${GR_JUPYTER_NOTEBOOK_IP}:~/
elif [[ "$1" == "tun" ]]; then
    ssh ubuntu@${GR_JUPYTER_NOTEBOOK_IP} -i ~/.ssh/gr-dev-instances-1.pem -L 8888:127.0.0.1:8888  -N -v -v -v
fi
