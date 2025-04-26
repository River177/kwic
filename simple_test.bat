@echo off
chcp 65001 > nul
echo 创建测试输入文件...
echo The quick brown fox jumps over the lazy dog > test_input.txt
echo Architecture styles are important for software design >> test_input.txt
echo KWIC system demonstrates different architectural approaches >> test_input.txt

echo 创建输出目录...
mkdir test_results 2>nul

echo 运行测试...
echo ----------------------------------------------
echo 测试架构风格: main_subroutine
if exist main_subroutine (
  cd main_subroutine
  cargo build
  cargo run < ..\test_input.txt > ..\test_results\main_subroutine.txt
  cd ..
  echo 结果已保存到 test_results\main_subroutine.txt
) else (
  echo 目录不存在，跳过测试
)

echo ----------------------------------------------
echo 测试架构风格: object_oriented
if exist object_oriented (
  cd object_oriented
  cargo build
  cargo run < ..\test_input.txt > ..\test_results\object_oriented.txt
  cd ..
  echo 结果已保存到 test_results\object_oriented.txt
) else (
  echo 目录不存在，跳过测试
)

echo ----------------------------------------------
echo 测试架构风格: pipe_filter
if exist pipe_filter (
  cd pipe_filter
  cargo build
  cargo run < ..\test_input.txt > ..\test_results\pipe_filter.txt
  cd ..
  echo 结果已保存到 test_results\pipe_filter.txt
) else (
  echo 目录不存在，跳过测试
)

echo ----------------------------------------------
echo 测试架构风格: repository
if exist repository (
  cd repository
  cargo build
  cargo run < ..\test_input.txt > ..\test_results\repository.txt
  cd ..
  echo 结果已保存到 test_results\repository.txt
) else (
  echo 目录不存在，跳过测试
)

echo ----------------------------------------------
echo 测试架构风格: communicating_sequential_processes
if exist communicating_sequential_processes (
  cd communicating_sequential_processes
  cargo build
  cargo run < ..\test_input.txt > ..\test_results\communicating_sequential_processes.txt
  cd ..
  echo 结果已保存到 test_results\communicating_sequential_processes.txt
) else (
  echo 目录不存在，跳过测试
)

echo ----------------------------------------------
echo 测试架构风格: event_driven
if exist event_driven (
  cd event_driven
  cargo build
  cargo run < ..\test_input.txt > ..\test_results\event_driven.txt
  cd ..
  echo 结果已保存到 test_results\event_driven.txt
) else (
  echo 目录不存在，跳过测试
)

echo ----------------------------------------------
echo 测试架构风格: interpreter
if exist interpreter (
  cd interpreter
  cargo build
  cargo run < ..\test_input.txt > ..\test_results\interpreter.txt
  cd ..
  echo 结果已保存到 test_results\interpreter.txt
) else (
  echo 目录不存在，跳过测试
)

echo ----------------------------------------------
echo 测试完成，按任意键查看结果...
pause > nul
type test_results\*.txt 