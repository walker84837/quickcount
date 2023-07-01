// Get references to the necessary elements
const textInput = document.getElementById('text-input') as HTMLTextAreaElement;
const wordCount = document.getElementById('word-count');
const letterCountExcludingSpaces = document.getElementById('letter-count-excluding-spaces');
const letterCountIncludingSpaces = document.getElementById('letter-count-including-spaces');

// Event listener to count words and letters on input change
textInput.addEventListener('input', () => {
	const text = textInput.value;
	const words = text.trim().split(/\s+/);
	const lettersExcludingSpaces = text.replace(/\s/g, '').length;
	const lettersIncludingSpaces = text.length;

	wordCount.textContent = `Word count: ${words.length}`;
	letterCountExcludingSpaces.textContent = `Letter count (excluding spaces): ${lettersExcludingSpaces}`;
	letterCountIncludingSpaces.textContent = `Letter count (including spaces): ${lettersIncludingSpaces}`;
});
