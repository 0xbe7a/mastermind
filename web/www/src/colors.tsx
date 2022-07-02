import React from 'react';

type ColorProps = {
  activeColor: string,
  colors: string[],
  onClick: (color: string) => void
}

export const Colors = (props: ColorProps) => {

  const allColors = props.colors.map((color) => {    
    const active = color === props.activeColor ? 'active' : '';  

    return (
      <div
        className={'color-holder ' + color + ' ' + active}
        key={color}
        onClick={() => { props.onClick(color) }} >
      </div>
      )
    })

  return (
    <div className='colors'>
      {allColors}
    </div>
    );
}