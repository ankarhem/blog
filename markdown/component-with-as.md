---
title: Polymorphic components with `as`
tags: [react, typescript]
excerpt: How to create a typed component that can render as an intrinsic element or another React component
---

## Example component

Minimal example of a component that can control styling but pass
the rest of the functionality to an underlying component

```tsx
import {
  ElementType,
  JSXElementConstructor,
  ReactNode,
  ComponentProps,
} from "react";

// Utility types
type ReactTag = keyof JSX.IntrinsicElements | JSXElementConstructor<any>;

type Props<T extends ReactTag, ControlledProps> = Omit<
  ComponentProps<T>,
  keyof ControlledProps
> &
  ControlledProps;

// Component prop types
type ButtonProps<T extends ElementType> = {
  as: T;
  children?: ReactNode;
  className?: string;
};

export const Button = <T extends ReactTag = "button">({
  as,
  className,
  children,
  ...rest
}: Props<T, ButtonProps<T>>) => {
  const Component: ReactTag = as || "button";

  return (
    <Component className="button" {...rest}>
      {children}
    </Component>
  );
};
```

## Usage

### Intrinsic element

```tsx
const ButtonButActuallyLink = () => {
  return (
    <Button as="a" href="https://google.com">
      Click me
    </Button>
  );
};
```

### React component

```tsx
interface CoolComponentProps {
  isCool: boolean;
  className?: string;
}

const CoolComponent = ({ isCool, className }: CoolComponentProps) => {
  return <div className={className}>{isCool ? "Cool" : "Not cool"}</div>;
};

const ButtonButActuallyCoolComponent = () => {
  return <Button as={CoolComponent} isCool />;
};
```
