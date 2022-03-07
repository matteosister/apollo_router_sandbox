defmodule Server3Web.Schema do
  use Absinthe.Schema
  use Absinthe.Federation.Schema

  @claim %{id: 10, vehicle: "AA000BB", paid: 200}

  query do
    field :multiply, non_null(:integer) do
      arg :a, non_null(:integer)
      arg :b, non_null(:integer)
      resolve fn _, args, _ ->
        {:ok, args.a * args.b}
      end
    end

    field :claim, non_null(:claim) do
      arg :id, non_null(:id)
      resolve fn _, args, _ ->
        {:ok, @claim}
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
        {:ok, %Server3Web.Quote{id: id, claims: if id == "1" do [@claim] else [] end}}
      end
    end
  end

  object :claim do
    key_fields("id")
    field :id, non_null(:id)
    field :vehicle, :string
    field :paid, :integer
  end
end

