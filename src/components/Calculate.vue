<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';

const calcResult = ref('');
const problem = ref('');

async function calculate(): Promise<void> {
    calcResult.value = await invoke('calculate', { problem: problem.value });
}
</script>

<template>
    <div class="card">
        <input id="math-input" v-model="problem" placeholder="Enter problem..." />
        <button type="button" @click="calculate()">Calculate</button>
    </div>

    <p>{{ calcResult }}</p>
</template>
