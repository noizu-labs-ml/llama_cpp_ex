defmodule LLamaCppTest do
  use ExUnit.Case

  test "Prediction" do
    {:ok, llama} = LLamaCpp.new("./test/models/tinyllama-1.1b-chat-v1.0.Q4_K_M.gguf")
    llama2 = llama
    Process.sleep(5000)
    x = LLamaCpp.predict(llama2, "Say Hello. Example'Hello'")
    assert x =~ "Goodbye"

    IO.inspect(llama2)
    #x = LLamaCpp.predict(llama, "Say Hello. Example'Hello'")
    #assert x =~ "Goodbye"
    #LLamaCpp.free_model(llama)

  end

end
