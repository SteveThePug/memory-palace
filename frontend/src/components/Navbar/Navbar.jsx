import React, { useState, useEffect } from 'react';
import { NavLink } from 'react-router-dom';
import { useDispatch, useSelector } from 'react-redux';
import { userLogOut, userSet } from '../../store/user.js';
import Login  from './Login.jsx';
import Modal from '../Modal.jsx'

export default function Navbar() {
  const user = useSelector((state) => state.user?.result);
  const dispatch = useDispatch();
  
  const [isLoginModalOpen, setIsLoginModalOpen] = useState(false);
  const openLoginModal = () => setIsLoginModalOpen(true);
  const closeLoginModal = () => setIsLoginModalOpen(false);

  useEffect(() => {
    const token = JSON.parse(localStorage.getItem('profile'));
    if (token) dispatch(userSet(token));
  }, [dispatch]);

  const logout = () => dispatch(userLogOut());

  return (
    <div className='flex justify-between p-4 bg-slate-100'>
      {/* Logo/Title */}
      <NavLink to="/" className='text-xl font-bold text-blue-700'>STP</NavLink>

      {/* User Info */}
      {user ? (
        <>
          <button onClick={logout} className='text-xl font-bold text-blue-700 hover:text-blue-900'>LOGOUT</button>
          <NavLink className='text-xl font-bold text-blue-700 hover:text-blue-900'>{user.username}</NavLink>
        </>
      ) : (
        <button onClick={openLoginModal} className='text-xl font-bold text-blue-700 hover:text-blue-900'>
          LOGIN
        </button>
      )}

      {/* Login Modal */}
      <Modal isOpen={isLoginModalOpen} onClose={closeLoginModal}>
        <Login onLoginSuccess={closeLoginModal} />
      </Modal>
    </div>
  );
}