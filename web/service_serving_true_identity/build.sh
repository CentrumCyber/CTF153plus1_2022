#!/bin/bash
docker buildx build  -t chall_ssti ./challenge/ && docker run -it -p 5000:5000  chall_ssti