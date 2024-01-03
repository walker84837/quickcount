let text_input = document.getElementById('text-input');
let word_count = document.getElementById('word-count');
let letter_count_excluding_spaces = document.getElementById('letter-count-excluding-spaces');
let letter_count_including_spaces = document.getElementById('letter-count-including-spaces');

text_input.addEventListener('input', function () {
	let text = text_input.value;
	let words = text.trim().split(/\s+/);
	let letters_excluding_spaces = text.replace(/\s/g, '').length;
	let letters_including_spaces = text.length;
	word_count.textContent = `Word count: ${words.length}`;
	letter_count_excluding_spaces.textContent = `Letter count (excluding spaces): ${letters_excluding_spaces}`;
	letter_count_including_spaces.textContent = `Letter count (including spaces): ${letters_including_spaces}`;
});
