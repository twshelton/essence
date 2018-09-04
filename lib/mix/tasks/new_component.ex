defmodule Mix.Tasks.NewComponent do
  use Mix.Task
  import Mix.Generator
  require Mix.Generator
  
  @doc """
    Generate basic flow files for new flows
  """
  def run(opts) do
    IO.inspect opts
    {[f: file, c: components], _files, _} = OptionParser.parse(opts, strict: [f: :string, c: :string])

    bindings = [
      owner_module: file |> Mix.Utils.camelize,
	  components: components,
	  rust_methods: components |> String.split(",")
    ]

    path = "native/" <> file <> "/src/"

    create_file(Path.join(path, "lib.rs1"), rust_template(bindings))
    create_file(Path.join(path, "#{file}.ex1"), rust_elixir_template(bindings))
  end
  def component_to_nif(components) do
	components
	|> Enum.reduce([], fn(c, acc) -> ["(\"#{c}\", 1, #{c}),\n"] ++ acc end)
  end
  def components_to_fn(components) do
	components
	|> Enum.map(fn(c) ->
		rust_fn_template([name: c])
	end)
  end
  def components_to_elixir(owner, components) do
	components
	|> Enum.map(fn(c) ->
		rust_elixir_def_template([name: c, module_owner: owner])
	end)
  end
  embed_template :rust_elixir_def, """
    def <%= @name %> (handle), do: Indy.<%= @owner_module %>.<%= @name %>(handle)
  """
  embed_template :rust_elixir, """
defmodule <%= @owner_module %> do
    use Rustler, otp_app: :essence, crate: "<%= @owner_wallet %>"

    # When your NIF is loaded, it will override this function.
    <%= components_to_elixir(@owner_module, @rust_methods) %>
end
  
  """
  embed_template :rust, """
#[macro_use] extern crate rustler;
#[macro_use] extern crate rustler_codegen;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate serde_json;

extern crate libc;
extern crate indy;

mod callbacks;
mod utils;
mod results;

use libc::c_char;
use std::ffi::CString;
use indy::api::ErrorCode;
use rustler::{Env, Term, NifResult, Encoder};
use results::{result_to_string, result_to_int, result_to_empty};
use indy::api::<%= @owner_module %>::{<%= @components %>};

mod atoms {
    rustler_atoms! {
        atom ok;
        atom error;
        //atom __true__ = "true";
        //atom __false__ = "false";
    }
}

rustler_export_nifs! {
    "Elixir.Indy.<%= @owner_module |> String.upcase %>" ,
	<%= component_to_nif(@rust_methods) %>,
    None
}

<%= components_to_fn(@rust_methods) %>

 """

  embed_template :rust_fn, """
pub fn <%= @name %><'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

    let handle: i32 = try!(args[0].decode());

    let (receiver, command_handle, cb) = callbacks::_closure_to_cb_ec();

    let err = indy_<%= @name %>(command_handle, handle, cb);

    let response = match result_to_empty(err, receiver) {
        Ok(()) => (atoms::ok(), format!("Success!")),
        Err(err) => (atoms::error(), format!("There was a problem: {:?}", err)),
    };

    Ok(response.encode(env))
}

  """
end
