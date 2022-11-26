import { useRef, useState } from 'react';
import './App.css';

type InputEvent = React.KeyboardEvent<HTMLInputElement>;

function App() {
  const inputRef = useRef(null);
  const [todos, setTodos] = useState([] as string[]);
  const addTodo = (todo: string) => {
    setTodos([...todos, todo]);
  };
  const onInputChange = (e: InputEvent) => {
    e.preventDefault();
    if (e.code === "Enter") {
      addTodo(e.currentTarget.value);
    } 
  };
  return (
    <div className="App">
      <main>
        <div className='modal'>
          <input ref={inputRef} type="text" onKeyUp={onInputChange} />
          <ol>
            {todos.map((todo) => <li>{todo}</li>)}
          </ol>
        </div>
      </main>
    </div>
  );
}

export default App;
