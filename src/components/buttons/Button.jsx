import { buttonStyle } from "../../values/buttonStyle";

function Button({ text, action, param }) {
  return (
    <button
      onClick={() => {
        action(param);
      }}
      className={buttonStyle}
    >
      {text}
    </button>
  );
}

export default Button;
