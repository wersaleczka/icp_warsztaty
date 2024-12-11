<template>
  <div class="container mx-auto flex flex-col items-center justify-center gap-16 px-2 bg-gray-900 w-screen">
    <Header />
    <Calculator />
  </div>
</template>

<script>
import Header from "./components/Header.vue";
import Calculator from "./components/Calculator.vue";

export default {
  name: "App",
  components: {
    Header,
    Calculator,
  },
};
</script>
