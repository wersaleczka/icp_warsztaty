<script setup>
import { ref } from "vue";
import { my_project_backend } from "declarations/my_project_backend/index";
let greeting = ref("");

async function handleSubmit(e) {
  e.preventDefault();
  const target = e.target;
  const name = target.querySelector("#name").value;
  await my_project_backend.greet(name).then((response) => {
    greeting.value = response;
  });
}
</script>

<template>
  <main>
    <img src="/logo2.svg" alt="DFINITY logo" />
    <br />
    <br />
    <form action="#" @submit="handleSubmit" class="flex flex-col gap-4 justify-center items-center">
      <label for="name" class="text-4xl">Enter your name: &nbsp;</label>
      <input id="name" alt="Name" type="text" class="border-2 border-violet-600 rounded-lg" />
      <button type="submit" class="text-violet-50 p-3 bg-gradient-to-r from-violet-600 to-violet-800 rounded-xl">
        Click Me!
      </button>
    </form>
    <section id="greeting">{{ greeting }}</section>
  </main>
</template>

<script>
export default {
  name: "Header",
};
</script>
