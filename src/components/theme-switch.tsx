import React, { useEffect, useState } from "react";

const ThemeSwitcher = (props: { theme: string; toggleTheme: () => void }) => {
  const { theme, toggleTheme } = props;
  const isDarkMode = theme === "dark";

  return (
    <div className="flex items-center justify-center bg-gray-100 dark:bg-gray-900 transition-colors duration-300">
      <button
        onClick={toggleTheme}
        className={`flex items-center justify-center p-1.5 rounded-full focus:outline-none transition-all duration-300 ease-in-out ${
          isDarkMode ? 'bg-yellow-400' : 'bg-gray-700'
        }`}
      >
        <span>
          {/* 亮模式图标 */}
          {isDarkMode ? (
            <svg
              className="w-4 h-4 text-white"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
              xmlns="http://www.w3.org/2000/svg"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth="2"
                d="M12 3v1m0 16v1m8.66-9h-1M4.34 12h-1m15.15-5.66l-.707.707M6.05 6.05l-.707.707m12.73 12.73l-.707-.707M6.05 17.95l-.707-.707M12 5a7 7 0 000 14 7 7 0 000-14z"
              />
            </svg>
          ) : (
            // 优化的暗模式月亮图标
            <svg
              className="w-4 h-4 text-white"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
              xmlns="http://www.w3.org/2000/svg"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth="2"
                d="M21 12.79A9 9 0 1111.21 3a7 7 0 109.79 9.79z"
              />
            </svg>
          )}
        </span>
      </button>
    </div>
  );
};

export default ThemeSwitcher;
