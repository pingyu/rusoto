#!/usr/bin/env bash
set -Eeu

# Test MinIO with S3 path-style addressing.

export AWS_ACCESS_KEY_ID=ANTN35UAENTS5UIAEATD
export AWS_SECRET_ACCESS_KEY=TtnuieannGt2rGuie2t8Tt7urarg5nauedRndrur
export DOMAIN=s3.us-east-1.rusoto.example.com
export S3_ENDPOINT="http://$DOMAIN:9000"
export S3_PROXY=http://127.0.0.1:9000
export S3_ADDRESSING_STYLE=path

GIT_ROOT=$(git rev-parse --show-toplevel)
cd "$GIT_ROOT/integration_tests"
./docker_test_run.py \
    --docker-image="minio/minio" \
    --docker-image="minio/minio:edge" \
    --port=9000 \
    --run-opt=-p=9000:9000 \
    --run-opt=--env=MINIO_ACCESS_KEY=$AWS_ACCESS_KEY_ID \
    --run-opt=--env=MINIO_SECRET_KEY=$AWS_SECRET_ACCESS_KEY \
    --run-opt=--env=MINIO_DOMAIN=$DOMAIN \
    --run-arg=server \
    --run-arg=/home/shared \
    -- cargo test --features s3,disable_minio_unsupported ---- --test-threads 1
