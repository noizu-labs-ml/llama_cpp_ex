defmodule LLamaCpp do
  use Rustler,
    otp_app: :llama_cpp,
    crate: :llamacpp

  def new(path), do: new(path, %LLamaCpp.ModelOptions{})
  def new(_path, _model_opts), do: :erlang.nif_error(:nif_not_loaded)
  def predict(llama, query), do: predict(llama, query, %LLamaCpp.PredictOptions{})
  def predict(_llama, _query, _options), do: :erlang.nif_error(:nif_not_loaded)
  def embeddings(llama, query), do: embeddings(llama, query, %LLamaCpp.PredictOptions{})
  def embeddings(_llama, _query, _options), do: :erlang.nif_error(:nif_not_loaded)
  def free_model(_llama), do: :erlang.nif_error(:nif_not_loaded)
end

defmodule DebugFoo do
  def go() do
      {:ok, bobby} = LLamaCpp.new("./test/models/tinyllama-1.1b-chat-v1.0.Q4_K_M.gguf")
      bb = LLamaCpp.predict(bobby, "Say Hello World") |> IO.inspect(label: :a)
      cc = LLamaCpp.predict(bobby, "Say Hello World") |> IO.inspect(label: :b)
      dd = LLamaCpp.predict(bobby, "Say Hello World3") |> IO.inspect(label: :c)
      ee = LLamaCpp.predict(bobby, "Say Hello World3") |> IO.inspect(label: :d)
      {bb,cc,dd, ee}
  end
end
