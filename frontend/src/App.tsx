import {
  Match,
  Suspense,
  Switch,
  createResource,
  createSignal,
} from "solid-js";
import {
  QueryClient,
  QueryClientProvider,
  createQuery,
} from "@tanstack/solid-query";

const queryClient = new QueryClient();

function App() {
  console.log();

  return (
    <QueryClientProvider client={queryClient}>
      <Demo />
    </QueryClientProvider>
  );
}

function Demo() {
  const [count, setCount] = createSignal(0);
  const query = createQuery(
    () => ["hello"],
    () => fetch("http://localhost:3000/api/hello").then((res) => res.text()),
  );

  return (
    <div class="flex flex-col items-center">
      <div class="py-12"></div>
      <h1 class="font-mono text-indigo-500 font-semibold text-2xl mb-8">
        Vite + Solid served by Rust!
      </h1>
      <div class="rounded-xl py-3 px-2 bg-slate-300 shadow-lg w-96">
        <div class="flex flex-col space-y-6">
          <p class="text-2xl">Hello from vite!</p>
          <p class="text-lg">
            <Switch>
              <Match when={query.isLoading}>Loading...</Match>
              <Match when={query.isError}>Error!</Match>
              <Match when={query.isSuccess}>{query.data}</Match>
            </Switch>
          </p>
          <button
            class="bg-green-500/80 rounded-xl py-3 px-2 hover:bg-green-500/70 active:bg-green-500 w-full font-bold uppercase text-white"
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
