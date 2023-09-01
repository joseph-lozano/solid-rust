import { Suspense, createResource, createSignal } from "solid-js";

function App() {
  const [count, setCount] = createSignal(0);

  const [message] = createResource(() =>
    fetch("/api/hello").then((res) => res.text()),
  );

  return (
    <div class="flex flex-col items-center">
      <div class="py-12"></div>
      <h1 class="font-mono text-primary font-semibold text-2xl mb-8">
        Vite + Solid served by Rust!
      </h1>
      <div class="card bg-base-100 shadow-lg w-96">
        <div class="card-body">
          <p>Hello from vite!</p>
          <Suspense fallback={<p>Loading...</p>}>
            <p>{message()}</p>
          </Suspense>
          <button
            class="btn w-full"
            onClick={() => setCount((count) => count + 1)}
          >
            count is {count()}
          </button>
        </div>
      </div>
    </div>
  );
}

export default App;
