import React, { useState } from 'react';
import { useDispatch } from 'react-redux';
import { postsCreate } from '../../store/posts.js';

export default function Form() {
  const dispatch = useDispatch();
  
  const [inputs, setInputs] = useState({ title: '', markdown: '' });
  
  const handleChange = (event) => {
    const name = event.target.name;
    const value = event.target.value;
    setInputs(values => ({...values, [name]: value}));
  }
  
  const handleFileChange = (e) => {
    const file = e.target.files[0];
    if (file) {
      const reader = new FileReader();
      reader.onload = (event) => {
        const fileText = event.target.result;
        setInputs((inputs) => ({
          ...inputs,
          markdown: fileText,
        }));
      };
      reader.readAsText(file);
    }
  };

  const handleSubmit = (event) => {
    event.preventDefault();
    dispatch(postsCreate({...inputs}));
  }
  
  return (
    <form 
      onSubmit={handleSubmit} 
      className="flex flex-col max-w-lg mx-auto p-4 space-y-4 bg-white shadow-md rounded-lg"
    >
      <input 
        type="text" 
        name="title" 
        value={inputs.title}
        onChange={handleChange}
        placeholder="Title"
        className="p-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
      />
      <textarea 
        name="markdown" 
        value={inputs.markdown}
        onChange={handleChange}
        placeholder="Write your markdown here..."
        className="p-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 h-40"
      />
      <div className="flex items-center space-x-2">
        <label className="flex-grow">
          <span className="sr-only">Choose file</span>
          <input 
            type="file" 
            accept='.md' 
            onChange={handleFileChange}
            className="block w-full text-sm text-slate-500
              file:mr-4 file:py-2 file:px-4
              file:rounded-full file:border-0
              file:text-sm file:font-semibold
              file:bg-violet-50 file:text-violet-700
              hover:file:bg-violet-100"
          />
        </label>
        <span className="text-sm text-gray-500">or</span>
      </div>
      <input 
        type="submit" 
        value="Submit"
        className="p-2 bg-blue-500 text-white rounded-md hover:bg-blue-600 transition-all cursor-pointer"
      />
    </form>
  );
}