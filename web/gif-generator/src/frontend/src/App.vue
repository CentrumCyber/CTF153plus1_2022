<script setup lang="ts">
import { ref } from "vue"
import { NButton } from "naive-ui"
import { NInput } from "naive-ui"
import axios from "axios"

const gif = ref("");
const imageToRender = ref("");
const errorMessage = ref("");

async function getGif() {
  if (gif.value === "") return;

  errorMessage.value = "";
  try {
    let response = await axios.get(`http://localhost:50015/api/get-gif?input=${gif.value}`);
    imageToRender.value = response.data.url;
  }
  catch {
    errorMessage.value = "Server error";
  }
}
</script>

<template>
  <main class="wrapper">
    <div class="title">
      <h1>Random Gif Generator</h1>
      <h2>Input a word to get a random gif of it!</h2>
    </div>
    <div class="content">
      <div class="form">
        <div class="input-wrapper">
          <n-input type="text" v-model:value="gif" placeholder="Awesome word" />
        </div>
        <n-button color="#5E81AC" type="primary" @click="getGif">Gimme a gif!</n-button>
        <n-button color="#BF616A" type="primary" @click="imageToRender=''">Clear</n-button>
      </div>
      <p class="error">{{ errorMessage }}</p>
      <div class="image">
        <img :src="imageToRender"  height="600" />
      </div>
    </div>
  </main>
</template>

<style scoped>
.wrapper {
  color: #D8DEE9;
}
.form {
  display: flex;
  align-items: center;
  justify-content: space-evenly;
  width: 25rem;
}
.content {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
}
.image {
  margin: 1rem;
}
.error {
  color: red;
}
</style>
