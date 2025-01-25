defmodule GrapheneTest do
  use ExUnit.Case
  doctest Graphene

  test "greets the world" do
    assert Graphene.hello() == :world
  end
end
