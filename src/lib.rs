#![feature(box_patterns)]
#![feature(iterator_try_reduce)]
#![allow(clippy::type_complexity, clippy::too_many_arguments)]
#![warn(
    clippy::as_underscore,
    clippy::bool_to_int_with_if,
    clippy::cargo_common_metadata,
    clippy::case_sensitive_file_extension_comparisons,
    clippy::cast_lossless,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::checked_conversions,
    clippy::clone_on_ref_ptr,
    clippy::cloned_instead_of_copied,
    clippy::cognitive_complexity,
    clippy::copy_iterator,
    clippy::create_dir,
    clippy::default_trait_access,
    clippy::deref_by_slicing,
    clippy::doc_link_with_quotes,
    clippy::doc_markdown,
    clippy::empty_enum,
    clippy::empty_line_after_outer_attr,
    clippy::empty_structs_with_brackets,
    clippy::enum_glob_use,
    clippy::equatable_if_let,
    clippy::exit,
    clippy::expl_impl_clone_on_copy,
    clippy::explicit_deref_methods,
    clippy::explicit_into_iter_loop,
    clippy::explicit_iter_loop,
    clippy::filetype_is_file,
    clippy::filter_map_next,
    clippy::flat_map_option,
    clippy::float_cmp,
    clippy::float_cmp_const,
    clippy::fn_params_excessive_bools,
    clippy::fn_to_numeric_cast_any,
    clippy::from_iter_instead_of_collect,
    clippy::future_not_send,
    clippy::get_unwrap,
    clippy::if_not_else,
    clippy::if_then_some_else_none,
    clippy::implicit_hasher,
    clippy::imprecise_flops,
    clippy::inconsistent_struct_constructor,
    clippy::index_refutable_slice,
    clippy::inefficient_to_string,
    clippy::invalid_upcast_comparisons,
    clippy::items_after_statements,
    clippy::iter_not_returning_iterator,
    clippy::iter_on_empty_collections,
    clippy::iter_on_single_items,
    clippy::iter_with_drain,
    clippy::large_digit_groups,
    clippy::large_stack_arrays,
    clippy::large_types_passed_by_value,
    clippy::linkedlist,
    clippy::lossy_float_literal,
    clippy::manual_assert,
    clippy::manual_instant_elapsed,
    clippy::manual_let_else,
    clippy::manual_ok_or,
    clippy::manual_string_new,
    clippy::many_single_char_names,
    clippy::map_err_ignore,
    clippy::map_unwrap_or,
    clippy::match_on_vec_items,
    clippy::mismatching_type_param_order,
    clippy::missing_const_for_fn,
    clippy::missing_enforced_import_renames,
    clippy::must_use_candidate,
    clippy::mut_mut,
    clippy::naive_bytecount,
    clippy::needless_bitwise_bool,
    clippy::needless_collect,
    clippy::needless_continue,
    clippy::needless_for_each,
    clippy::needless_pass_by_value,
    clippy::negative_feature_names,
    clippy::non_ascii_literal,
    clippy::non_send_fields_in_send_ty,
    clippy::or_fun_call,
    clippy::range_minus_one,
    clippy::range_plus_one,
    clippy::rc_buffer,
    clippy::redundant_closure_for_method_calls,
    clippy::redundant_else,
    clippy::redundant_feature_names,
    clippy::redundant_pub_crate,
    clippy::ref_option_ref,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::return_self_not_must_use,
    clippy::same_functions_in_if_condition,
    clippy::semicolon_if_nothing_returned,
    clippy::separated_literal_suffix,
    clippy::significant_drop_in_scrutinee,
    clippy::single_match_else,
    clippy::str_to_string,
    clippy::string_add,
    clippy::string_add_assign,
    clippy::string_slice,
    clippy::struct_excessive_bools,
    clippy::suboptimal_flops,
    clippy::suspicious_operation_groupings,
    clippy::suspicious_xor_used_as_pow,
    clippy::trailing_empty_array,
    clippy::trait_duplication_in_bounds,
    clippy::transmute_ptr_to_ptr,
    clippy::transmute_undefined_repr,
    clippy::trivial_regex,
    clippy::trivially_copy_pass_by_ref,
    clippy::try_err,
    clippy::type_repetition_in_bounds,
    clippy::undocumented_unsafe_blocks,
    clippy::unicode_not_nfc,
    clippy::uninlined_format_args,
    clippy::unnecessary_join,
    clippy::unnecessary_self_imports,
    clippy::unneeded_field_pattern,
    clippy::unnested_or_patterns,
    clippy::unreadable_literal,
    clippy::unsafe_derive_deserialize,
    clippy::unused_async,
    clippy::unused_peekable,
    clippy::unused_rounding,
    clippy::unused_self,
    clippy::unwrap_in_result,
    clippy::use_self,
    clippy::useless_let_if_seq,
    clippy::verbose_bit_mask,
    clippy::verbose_file_reads
)]
#![deny(
    clippy::derive_partial_eq_without_eq,
    clippy::match_bool,
    clippy::mem_forget,
    clippy::mutex_atomic,
    clippy::mutex_integer,
    clippy::nonstandard_macro_braces,
    clippy::path_buf_push_overwrite,
    clippy::rc_mutex,
    clippy::wildcard_dependencies
)]

pub mod ast;
pub mod errors;
pub mod file_importer;
pub mod interpreter;
pub mod lexer;
pub mod parser;
pub mod primitives;
pub mod repl;
pub mod types;

use std::{path::Path, sync::Arc, time::Instant};

use errors::{ZError, ZResult};
use itertools::Either;
use smol_str::SmolStr;
use tracing::{debug, info, trace};

use crate::{
    ast::{Ast, AstData, Reconstruct},
    file_importer::{import_file, register_input},
    interpreter::interpret_asts,
    lexer::lex,
    parser::parse_token_list,
    types::{
        r#type::Type,
        sym_table::{InterpretSymTable, TypecheckSymTable},
        value::Value,
    },
};

pub fn compile(
    file: &Either<&Path, (SmolStr, String)>,
    ty_symt: &mut TypecheckSymTable,
) -> ZResult<Vec<Ast>> {
    /*if ty_symt.out.verbosity() == 0 {
        return gen_instructions(parse_token_list(lex(input, filename)?)?, ty_symt);
    }*/
    // TODO --stats flag

    let (input, filename) = match &file {
        Either::Left(p) => (import_file(p), SmolStr::from(p.to_string_lossy())),
        Either::Right((name, input)) => (register_input(name, input), name.to_owned()),
    };

    info!("Lexing");
    let lex_start = Instant::now();
    let lexed = lex((*input).to_owned(), filename)?;
    let lex_time = lex_start.elapsed().as_micros();
    trace!("{lexed:#?}");

    info!("Parsing");
    let parse_start = Instant::now();
    let mut parsed = parse_token_list(lexed)?;
    let parse_time = parse_start.elapsed().as_micros();
    trace!("{parsed:#?}");
    debug!("{}", parsed.reconstruct());

    info!("Desugaring");
    let desugar_start = Instant::now();
    for ele in &mut parsed {
        ele.desugar()?;
    }
    let desugar_time = desugar_start.elapsed().as_micros();
    trace!("{parsed:#?}");
    debug!("{}", parsed.reconstruct());

    info!("Typechecking");
    let typecheck_start = Instant::now();
    for ele in &mut parsed {
        ele.typecheck(ty_symt)?;
    }
    let typecheck_time = typecheck_start.elapsed().as_micros();
    trace!("{parsed:#?}");

    info!("Stats:");
    info!("Lexing time: {lex_time}\u{b5}s");
    info!("Parsing time: {parse_time}\u{b5}s");
    info!("Desugar time: {desugar_time}\u{b5}s");
    info!("Typecheck time: {typecheck_time}\u{b5}s");
    info!(
        "Total time: {}\u{b5}s\n",
        lex_time + parse_time + desugar_time + typecheck_time
    );

    Ok(parsed)
}

pub fn interpret(input: &Vec<Ast>, val_symt: &mut InterpretSymTable) -> ZResult<i32> {
    /*if val_symt.out.verbosity() == 0 {
        return interpret_asts(input, val_symt);
    }*/
    // TODO --stats flag
    info!("Interpreting");
    let interpret_start = Instant::now();
    let exit_code = interpret_asts(input, val_symt)?;
    let interpret_time = interpret_start.elapsed().as_micros();
    info!("Exited with code {exit_code}");
    info!("Stats");
    info!("Interpreting time: {interpret_time}\u{b5}s");
    Ok(exit_code)
}
