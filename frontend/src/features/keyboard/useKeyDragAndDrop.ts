import { ref } from 'vue';

import { useKeyboardStore } from './store';

export function useKeyDragAndDrop() {
  const keyboard = useKeyboardStore();
  const draggedIndex = ref<number | null>(null);

  function onDragStart(index: number) {
    draggedIndex.value = index;
  }

  function onDragOver(event: DragEvent) {
    event.preventDefault();
  }

  function onDrop(index: number) {
    if (draggedIndex.value !== null && draggedIndex.value !== index) {
      keyboard.reorderKey(draggedIndex.value, index);
    }
    draggedIndex.value = null;
  }

  return { draggedIndex, onDragStart, onDragOver, onDrop };
}
