function practice(nums: number[], index: number): number {
  return (nums[index] ?? index) * 5;
}

console.log(practice([], 5));
