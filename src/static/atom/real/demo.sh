#!/bin/bash

# 指定目录路径

# 遍历目录下的文件
for file in `ls`; do
    # 提取文件名
    filename=$(basename "$file")
    # 删除'look'前缀
    new_filename="${filename#color}"
    # 构建新的文件路径
    new_filepath="./$new_filename"
    # 重命名文件
    mv "$file" "$new_filepath"
done

