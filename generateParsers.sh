#! /bin/sh
flex --nounistd -osrc/compiler/glsl/glcpp/glcpp-lex.c src/compiler/glsl/glcpp/glcpp-lex.l
flex --nounistd -osrc/compiler/glsl/glsl_lexer.cpp src/compiler/glsl/glsl_lexer.ll
bison -v -o "src/compiler/glsl/glcpp/glcpp-parse.c" -p "glcpp_parser_" --defines=src/compiler/glsl/glcpp/glcpp-parse.h src/compiler/glsl/glcpp/glcpp-parse.y
bison -v -o "src/compiler/glsl/glsl_parser.cpp" -p "_mesa_glsl_" --defines=src/compiler/glsl/glsl_parser.h src/compiler/glsl/glsl_parser.yy

python "src/compiler/glsl/ir_expression_operation.py" "enum" >src/compiler/glsl/ir_expression_operation.h
python "src/compiler/glsl/ir_expression_operation.py" "strings" >src/compiler/glsl/ir_expression_operation_strings.h
python "src/compiler/glsl/ir_expression_operation.py" "constant" >src/compiler/glsl/ir_expression_operation_constant.h

python src/util/format_srgb.py >src/util/format_srgb.c
python src/util/format/u_format_table.py src/util/format/u_format.csv --header >src/util/format/u_format_pack.h
python src/util/format/u_format_table.py src/util/format/u_format.csv >src/util/format/u_format_table.c
