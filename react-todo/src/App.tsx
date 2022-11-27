import { useRef, useState } from 'react';
import './App.css';

type InputEvent = React.KeyboardEvent<HTMLInputElement>;

enum Filter {
  ACTIVE = 'active',
  ALL = 'all',
  DONE = 'done',
}

type Todo = {
  content: string;
  isDone: boolean;
}

function App() {
  const inputRef = useRef(null);
  const [activeButton, setActiveButton] = useState(Filter.ALL);
  const [todos, setTodos] = useState([] as Todo[]);
  const addTodo = (newTodoContent: string) => {
    const todo = {
      content: newTodoContent,
      isDone: false,
    };
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
          <div>{activeButton}</div>
          <ol>
            {todos.map((todo) => <li>{todo.content}</li>)}
          </ol>
          <div className="button-list">
            <button onClick={() => setActiveButton(Filter.ACTIVE)} type='button' className={activeButton === "active" ? 'active' : ""}>Active</button>
            <button  onClick={() => setActiveButton(Filter.DONE)} type='button' className={activeButton === "done" ? 'active' : ""}>Done</button>
            <button  onClick={() => setActiveButton(Filter.ALL)} type='button' className={activeButton === "all" ? 'active' : ""}>All</button>
          </div>
        </div>
      </main>
    </div>
  );
}

export default App;
