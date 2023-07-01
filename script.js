// Get references to the necessary elements
let textInput = document.getElementById('text-input');
let wordCount = document.getElementById('word-count');
let letterCountExcludingSpaces = document.getElementById('letter-count-excluding-spaces');
let letterCountIncludingSpaces = document.getElementById('letter-count-including-spaces');
// Event listener to count words and letters on input change
textInput.addEventListener('input', function () {
    let text = textInput.value;
    let words = text.trim().split(/\s+/);
    let lettersExcludingSpaces = text.replace(/\s/g, '').length;
    let lettersIncludingSpaces = text.length;
    wordCount.textContent = "Word count: ".concat(words.length);
    letterCountExcludingSpaces.textContent = "Letter count (excluding spaces): ".concat(lettersExcludingSpaces);
    letterCountIncludingSpaces.textContent = "Letter count (including spaces): ".concat(lettersIncludingSpaces);
});
