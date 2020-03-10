use bindgen;
use cc;

use std::path::PathBuf;
use std::env;

fn main() {
    let host = env::var("HOST").unwrap();
    let target = env::var("TARGET").unwrap();
    assert_eq!(host, target, "glslopt should always be built as a build dependency");

    println!("{}", "begin env");
    for (key, value) in env::vars() {
        println!("{}: {}", key, value);
    }
    println!("{}", "end env");

    // Unset CFLAGS as they are probably intended for a target build,
    // so may break building this as a build dependency.
    env::remove_var(format!("CFLAGS_{}", &target));
    env::remove_var(format!("CXXFLAGS_{}", &target));
    env::remove_var(format!("CFLAGS_{}", target.replace("-", "_")));
    env::remove_var(format!("CXXFLAGS_{}", target.replace("-", "_")));
    env::remove_var("CFLAGS");
    env::remove_var("CXXFLAGS");

    // This has been set to override --target= to help windows cross builds,
    // but breaks this building as a build dependency.
    env::remove_var("BINDGEN_EXTRA_CLANG_ARGS");

    cc::Build::new()
        .warnings(false)
        .extra_warnings(false)
        .include("include")
        .include("src")
        .include("src/mesa")
        .file("src/glsl/glcpp/glcpp-lex.c")
        .file("src/glsl/glcpp/glcpp-parse.c")
        .file("src/glsl/glcpp/pp.c")
        .file("src/glsl/strtod.c")
        .file("src/util/hash_table.c")
        .file("src/util/ralloc.c")
        .compile("glcpp-library");

    cc::Build::new()
        .warnings(false)
        .extra_warnings(false)
        .include("include")
        .include("src")
        .include("src/mesa")
        .file("src/mesa/main/imports.c")
        .file("src/mesa/program/prog_hash_table.c")
        .file("src/mesa/program/symbol_table.c")
        .compile("mesa");

    cc::Build::new()
        .cpp(true)
        .warnings(false)
        .extra_warnings(false)
        .include("include")
        .include("src")
        .include("src/mesa")
        .file("src/glsl/ast_array_index.cpp")
        .file("src/glsl/ast_expr.cpp")
        .file("src/glsl/ast_function.cpp")
        .file("src/glsl/ast_to_hir.cpp")
        .file("src/glsl/ast_type.cpp")
        .file("src/glsl/builtin_functions.cpp")
        .file("src/glsl/builtin_types.cpp")
        .file("src/glsl/builtin_variables.cpp")
        .file("src/glsl/glsl_lexer.cpp")
        .file("src/glsl/glsl_optimizer.cpp")
        .file("src/glsl/glsl_parser_extras.cpp")
        .file("src/glsl/glsl_parser.cpp")
        .file("src/glsl/glsl_symbol_table.cpp")
        .file("src/glsl/glsl_types.cpp")
        .file("src/glsl/hir_field_selection.cpp")
        .file("src/glsl/ir_basic_block.cpp")
        .file("src/glsl/ir_builder.cpp")
        .file("src/glsl/ir_clone.cpp")
        .file("src/glsl/ir_constant_expression.cpp")
        .file("src/glsl/ir_equals.cpp")
        .file("src/glsl/ir_expression_flattening.cpp")
        .file("src/glsl/ir_function_can_inline.cpp")
        .file("src/glsl/ir_function_detect_recursion.cpp")
        .file("src/glsl/ir_function.cpp")
        .file("src/glsl/ir_hierarchical_visitor.cpp")
        .file("src/glsl/ir_hv_accept.cpp")
        .file("src/glsl/ir_import_prototypes.cpp")
        .file("src/glsl/ir_print_glsl_visitor.cpp")
        .file("src/glsl/ir_print_metal_visitor.cpp")
        .file("src/glsl/ir_print_visitor.cpp")
        .file("src/glsl/ir_rvalue_visitor.cpp")
        .file("src/glsl/ir_stats.cpp")
        .file("src/glsl/ir_unused_structs.cpp")
        .file("src/glsl/ir_validate.cpp")
        .file("src/glsl/ir_variable_refcount.cpp")
        .file("src/glsl/ir.cpp")
        .file("src/glsl/link_atomics.cpp")
        .file("src/glsl/link_functions.cpp")
        .file("src/glsl/link_interface_blocks.cpp")
        .file("src/glsl/link_uniform_block_active_visitor.cpp")
        .file("src/glsl/link_uniform_blocks.cpp")
        .file("src/glsl/link_uniform_initializers.cpp")
        .file("src/glsl/link_uniforms.cpp")
        .file("src/glsl/link_varyings.cpp")
        .file("src/glsl/linker.cpp")
        .file("src/glsl/loop_analysis.cpp")
        .file("src/glsl/loop_controls.cpp")
        .file("src/glsl/loop_unroll.cpp")
        .file("src/glsl/lower_clip_distance.cpp")
        .file("src/glsl/lower_discard_flow.cpp")
        .file("src/glsl/lower_discard.cpp")
        .file("src/glsl/lower_if_to_cond_assign.cpp")
        .file("src/glsl/lower_instructions.cpp")
        .file("src/glsl/lower_jumps.cpp")
        .file("src/glsl/lower_mat_op_to_vec.cpp")
        .file("src/glsl/lower_named_interface_blocks.cpp")
        .file("src/glsl/lower_noise.cpp")
        .file("src/glsl/lower_offset_array.cpp")
        .file("src/glsl/lower_output_reads.cpp")
        .file("src/glsl/lower_packed_varyings.cpp")
        .file("src/glsl/lower_packing_builtins.cpp")
        .file("src/glsl/lower_ubo_reference.cpp")
        .file("src/glsl/lower_variable_index_to_cond_assign.cpp")
        .file("src/glsl/lower_vec_index_to_cond_assign.cpp")
        .file("src/glsl/lower_vec_index_to_swizzle.cpp")
        .file("src/glsl/lower_vector_insert.cpp")
        .file("src/glsl/lower_vector.cpp")
        .file("src/glsl/lower_vertex_id.cpp")
        .file("src/glsl/opt_algebraic.cpp")
        .file("src/glsl/opt_array_splitting.cpp")
        .file("src/glsl/opt_constant_folding.cpp")
        .file("src/glsl/opt_constant_propagation.cpp")
        .file("src/glsl/opt_constant_variable.cpp")
        .file("src/glsl/opt_copy_propagation_elements.cpp")
        .file("src/glsl/opt_copy_propagation.cpp")
        .file("src/glsl/opt_cse.cpp")
        .file("src/glsl/opt_dead_builtin_variables.cpp")
        .file("src/glsl/opt_dead_builtin_varyings.cpp")
        .file("src/glsl/opt_dead_code_local.cpp")
        .file("src/glsl/opt_dead_code.cpp")
        .file("src/glsl/opt_dead_functions.cpp")
        .file("src/glsl/opt_flatten_nested_if_blocks.cpp")
        .file("src/glsl/opt_flip_matrices.cpp")
        .file("src/glsl/opt_function_inlining.cpp")
        .file("src/glsl/opt_if_simplification.cpp")
        .file("src/glsl/opt_minmax.cpp")
        .file("src/glsl/opt_noop_swizzle.cpp")
        .file("src/glsl/opt_rebalance_tree.cpp")
        .file("src/glsl/opt_redundant_jumps.cpp")
        .file("src/glsl/opt_structure_splitting.cpp")
        .file("src/glsl/opt_swizzle_swizzle.cpp")
        .file("src/glsl/opt_tree_grafting.cpp")
        .file("src/glsl/opt_vectorize.cpp")
        .file("src/glsl/s_expression.cpp")
        .file("src/glsl/standalone_scaffolding.cpp")
        .compile("glsl_optimizer");

    let bindings = bindgen::Builder::default()
        .header("wrapper.hpp")
        .generate()
        .expect("Unable to generate bindings");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
