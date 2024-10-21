import { useEffect, useRef, useState } from "react";


export default function useHeight(defaultHeight: number = 0) {
  const [height, setHeight] = useState(defaultHeight);
  const ref = useRef<HTMLDivElement>(null);
  useEffect(() => {
    const handleResize = () => {
      if (ref.current) {
        setHeight(ref.current.offsetHeight);
      }
    };

    handleResize(); // Set initial height
    window.addEventListener('resize', handleResize);

    return () => {
      window.removeEventListener('resize', handleResize);
    };
  }, [])

  return { height, ref };
}
