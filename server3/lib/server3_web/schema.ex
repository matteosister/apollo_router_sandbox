defmodule Server3Web.Schema do
  use Absinthe.Schema
  use Absinthe.Federation.Schema

  query do
    field :multiply, non_null(:integer) do
      arg :a, non_null(:integer)
      arg :b, non_null(:integer)
      resolve fn  end_, args, _ ->
        {:ok, args.a * args.b}
      end
    end
  end

  @desc "Quote"
  object :quote do
    key_fields("id")
    extends()
    field :id, non_null(:id) do
      external()
    end
    field :claims, list_of(:claim)

    field(:_resolve_reference, :quote) do
      resolve fn _, %{__typename: "Quote", id: id}, _ ->
        {:ok, %Server3Web.Quote{id: id, claims: if id == "1" do [%{id: 10, vehicle: "AA000BB"}] else [] end}}
      end
    end
  end

  object :claim do
    field :id, :id
    field :vehicle, :string
  end
end

