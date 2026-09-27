#!/bin/bash

#../tools/protoc-36.2-linux-x86_64/bin/protoc ast_engine.proto --python_out=../mcpc/src/xy/ai/mcpc/tools/ast/rpc --proto_path=./
python -m grpc_tools.protoc \
  -I. \
  --python_out=../mcpc/src/xy/ai/mcpc/tools/ast/rpc \
  --grpc_python_out=../mcpc/src/xy/ai/mcpc/tools/ast/rpc \
  ast_engine.proto