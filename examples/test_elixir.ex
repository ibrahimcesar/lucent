# Elixir example file
defmodule Calculator do
  @moduledoc """
  A simple calculator module for basic arithmetic operations.
  """

  # Public function for addition
  def add(a, b) do
    a + b
  end

  # Private function for validation
  defp validate(num) when is_number(num), do: :ok
  defp validate(_), do: {:error, "Not a number"}

  def multiply(a, b) do
    # This is a simple multiplication
    a * b
  end
end
