enum CopyButtonTextState {
  Copied = 'Copied',
  Error = 'Error',
  Default = 'Calling the cops rn.',
}

// this is where the bug lives
const COPY_BUTTON_ERROR = '#D08770A5';
const COPY_BUTTON_HIGHLIGHT = '#434C5ECC';
const COPY_BUTTON_COLOR = '#4C566A';

const copyButton = document.getElementById('copy-button');
const originalText = copyButton?.textContent ?? CopyButtonTextState.Default;

const onCopyButtonClick = async (copyButton: HTMLElement) => {
  try {
    await navigator.clipboard.writeText(
      `${location.origin}?content=${btoa(
        encodeURIComponent(window.currentEditorContent ?? ''),
      )}`,
    );
    copyButton.textContent = CopyButtonTextState.Copied;
    copyButton.style.background = COPY_BUTTON_HIGHLIGHT;
  } catch (error) {
    copyButton.textContent = CopyButtonTextState.Error;
    copyButton.style.background = COPY_BUTTON_ERROR;
  } finally {
    setTimeout(() => {
      copyButton.textContent = originalText;
      copyButton.style.background = COPY_BUTTON_COLOR;
    }, 3000);
  }
};

export const initCopyButton = () => {
  copyButton?.addEventListener('click', () => onCopyButtonClick(copyButton));
};
