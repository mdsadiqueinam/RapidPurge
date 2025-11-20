<script setup>
import { formatBytes } from "@root/utils/bytes";

const { t } = useI18n();

const { scanData } = useFlashClean();

const formattedJunkFound = computed(() => {
  const total = scanData.value?.junkFound;
  return typeof total === "number" ? formatBytes(total) : null;
});

// Flatten the mixed node/sub-category payloads into a template-friendly shape.
const categorySections = computed(() => {
  const nodes = scanData.value?.nodes ?? [];
  return nodes.map((category) => ({
    id: category.categoryId,
    label: t(category.categoryId),
    size: formatBytes(category.size ?? 0),
    entries: buildEntries(category.nodes),
  }));
});

const buildEntries = (nodeGroup) => {
  if (!nodeGroup?.data?.length) return [];

  if (nodeGroup.event === "nodes") {
    return nodeGroup.data.map((item) => ({
      key: item.path ?? item.name,
      name: item.name ?? t("Unknown"),
      path: item.path,
      size: formatBytes(item.size ?? 0),
      type: "node",
      children: [],
    }));
  }

  if (nodeGroup.event === "subCategories") {
    return nodeGroup.data.map((subCategory) => ({
      key: subCategory.categoryId,
      name: t(subCategory.categoryId),
      size: formatBytes(subCategory.size ?? 0),
      type: "subcategory",
      children: buildEntries(subCategory.nodes),
    }));
  }

  return [];
};
</script>

<template>
  <div class="flex h-full flex-col gap-4 overflow-hidden">
    <div
      v-if="!scanData || !scanData.nodes?.length"
      class="flex grow flex-col items-center justify-center gap-2 rounded-2xl border border-border/40 text-center text-main-selected-text"
    >
      <p class="text-lg font-semibold">
        {{ t("Awaiting scan data") }}
      </p>
      <p class="max-w-sm text-sm">
        {{ t("Run a scan to see detailed categories, paths, and sizes here.") }}
      </p>
    </div>

    <div v-else class="flex grow flex-col gap-6 overflow-auto pr-1">
      <section class="rounded-2xl bg-sidebar/40 px-6 py-4">
        <p class="text-sm text-main-selected-text">
          {{ t("Total Junk Found") }}
        </p>
        <p class="text-3xl font-bold text-primary">
          {{ formattedJunkFound }}
        </p>
      </section>

      <section
        v-for="category in categorySections"
        :key="category.id"
        class="rounded-2xl border border-border/40"
      >
        <header
          class="flex flex-wrap items-center justify-between gap-3 border-b border-border/40 px-6 py-4"
        >
          <div>
            <p class="text-xs uppercase tracking-wide text-main-selected-text">
              {{ t("Category") }}
            </p>
            <p class="text-lg font-semibold">
              {{ category.label }}
            </p>
          </div>
          <div class="text-right">
            <p class="text-xs uppercase tracking-wide text-main-selected-text">
              {{ t("Size") }}
            </p>
            <p class="text-xl font-bold text-primary">
              {{ category.size }}
            </p>
          </div>
        </header>

        <ul class="divide-y divide-border/40">
          <li
            v-for="entry in category.entries"
            :key="entry.key"
            class="px-6 py-4"
          >
            <div class="flex flex-wrap items-start gap-4">
              <div class="flex-1 min-w-[200px]">
                <p class="font-medium">
                  {{ entry.name }}
                </p>
                <p
                  v-if="entry.path"
                  class="text-xs text-main-selected-text break-all"
                >
                  {{ entry.path }}
                </p>
              </div>
              <p class="text-sm font-semibold text-primary">
                {{ entry.size }}
              </p>
            </div>

            <ul
              v-if="entry.children.length"
              class="mt-3 space-y-3 border-l border-border/30 pl-4"
            >
              <li v-for="child in entry.children" :key="child.key">
                <div class="flex flex-wrap items-start gap-4">
                  <div class="flex-1 min-w-[200px]">
                    <p class="text-sm font-medium">
                      {{ child.name }}
                    </p>
                    <p
                      v-if="child.path"
                      class="text-xs text-main-selected-text break-all"
                    >
                      {{ child.path }}
                    </p>
                  </div>
                  <p class="text-sm font-semibold text-primary">
                    {{ child.size }}
                  </p>
                </div>
              </li>
            </ul>
          </li>
        </ul>
      </section>
    </div>
  </div>
</template>
