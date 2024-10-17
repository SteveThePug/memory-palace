import React, { useState } from 'react';
import { GoogleLogin } from '@react-oauth/google';
import { userSignIn, userSignUp } from '../../store/user.js'
import { useDispatch } from 'react-redux';


export default function Login({onLoginSuccess}) {
  const dispatch = useDispatch();
  // State
  const [isSignup, setIsSignup] = useState(false);
  const [googleUser, setGoogleUser] = useState('yo');
  const [inputs, setInputs] = useState({username: '', password:'', email:'', confirmPassword:''});
  // Handlers
  const handleSubmit = (e) => {
    e.preventDefault();
    
    const {username, email, password, confirmPassword} = inputs;
    
    if (isSignup) {
      console.log('Signing up...');
      if (password === confirmPassword) {
        dispatch(userSignUp({username, email, password}));
        onLoginSuccess()  
      }
      else {
        console.log("Passwords don't match") 
      }
    } 
    else {
      console.log('Signing in...');
      dispatch(userSignIn({username, email, password}));
      onLoginSuccess()  
    }
  };
  const handleChange = (event) => {
    const name = event.target.name;
    const value = event.target.value;
    setInputs(values => ({...values, [name]: value}))
  };
  const switchMode = () => {
    setIsSignup((prevIsSignup) => !prevIsSignup);
  };
  const onGoogleSuccess = (res) => {
    setGoogleUser(res.googleId);
  };

  return (    
    <div className="max-w-md mx-auto p-6 bg-white shadow-md rounded-lg">
      <h1 className="text-2xl font-bold mb-6 text-center">
        {isSignup ? 'Sign Up' : 'Sign In'}
      </h1>
    

      <form onSubmit={handleSubmit}>
        {/* Username */}
        <div className="mb-4">
          <label htmlFor="username" className="block text-sm font-medium text-gray-700">
            Username
          </label>
          <input 
            id="username"
            name="username"
            type="text"
            onChange={handleChange}
            className="mt-1 block w-full p-2 border border-gray-300 rounded-md"
            required
          />
        </div>

        {/* Email (only for signup) */}
        {
          <div className="mb-4">
            <label htmlFor="email" className="block text-sm font-medium text-gray-700">
              Email
            </label>
            <input 
              id="email"
              name="email"
              type="email"
              onChange={handleChange}
              className="mt-1 block w-full p-2 border border-gray-300 rounded-md"
            />
          </div>
        }

        {/* Password */}
        <div className="mb-4">
          <label htmlFor="password" className="block text-sm font-medium text-gray-700">
            Password
          </label>
          <input 
            id="password"
            name="password"
            type="password"
            onChange={handleChange}
            className="mt-1 block w-full p-2 border border-gray-300 rounded-md"
            required
          />
        </div>

        {/* Confirm Password (only for signup) */}
        {isSignup && (
          <div className="mb-4">
            <label htmlFor="confirmPassword" className="block text-sm font-medium text-gray-700">
              Confirm Password
            </label>
            <input 
              id="confirmPassword"
              name="confirmPassword"
              type="password"
              onChange={handleChange}
              className="mt-1 block w-full p-2 border border-gray-300 rounded-md"
              required
            />
          </div>
        )}

        {/* Submit Button */}
        <button
          type="submit"
          className="w-full bg-blue-600 text-white p-2 rounded-md hover:bg-blue-700 transition"
        >
          {isSignup ? 'Sign Up' : 'Sign In'}
        </button>

        {/* Switch between Sign In/Sign Up */}
        <p className="mt-4 text-center">
          {isSignup ? 'Already have an account?' : 'Don’t have an account?'}{' '}
          <button 
            type="button" 
            onClick={switchMode} 
            className="text-blue-500 underline"
          >
            {isSignup ? 'Sign In' : 'Sign Up'}
          </button>
        </p>

        {/* Google Authentication */}
        <GoogleLogin
          onSuccess={onGoogleSuccess}
          onError={() => {
            console.log('Login Failed');
          }}
        />
      </form>
    </div>
  );
}