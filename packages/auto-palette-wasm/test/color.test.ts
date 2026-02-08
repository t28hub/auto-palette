import { Color } from '@auto-palette/wasm';
import { describe, expect } from 'vitest';

describe('@auto-palette/wasm/color', () => {
  describe('isLight', () => {
    it('should return true for a light color', () => {
      // Act
      const color = Color.fromHexString('#FFFFFF');
      const actual = color.isLight();

      // Assert
      expect(actual).toBeTruthy();
    });

    it('should return false for a dark color', () => {
      // Act
      const color = Color.fromHexString('#000000');
      const actual = color.isLight();

      // Assert
      expect(actual).toBeFalsy();
    });
  });

  describe('isDark', () => {
    it('should return true for a dark color', () => {
      // Act
      const color = Color.fromHexString('#000000');
      const actual = color.isDark();

      // Assert
      expect(actual).toBeTruthy();
    });

    it('should return false for a light color', () => {
      // Act
      const color = Color.fromHexString('#ffffff');
      const actual = color.isDark();

      // Assert
      expect(actual).toBeFalsy();
    });
  });

  describe('toRGB', () => {
    it('should convert a color to RGB', () => {
      // Act
      const color = Color.fromHexString('#ff0080');
      const actual = color.toRGB();

      // Assert
      expect(actual).toEqual({
        r: expect.any(Number),
        g: expect.any(Number),
        b: expect.any(Number),
      });
      expect(actual).toEqual({ r: 255, g: 0, b: 128 });
    });
  });

  describe('toCMYK', () => {
    it('should convert a color to CMYK', () => {
      // Act
      const color = Color.fromHexString('#ff0080');
      const actual = color.toCMYK();

      // Assert
      expect(actual).toEqual({
        c: expect.any(Number),
        m: expect.any(Number),
        y: expect.any(Number),
        k: expect.any(Number),
      });
      expect(actual.c).toBeCloseTo(0.0);
      expect(actual.m).toBeCloseTo(1.0);
      expect(actual.y).toBeCloseTo(0.498039);
      expect(actual.k).toBeCloseTo(0.0);
    });
  });

  describe('toHSL', () => {
    it('should convert a color to HSL', () => {
      // Act
      const color = Color.fromHexString('#ff0080');
      const actual = color.toHSL();

      // Assert
      expect(actual).toEqual({
        h: expect.any(Number),
        s: expect.any(Number),
        l: expect.any(Number),
      });
      expect(actual.h).toBeCloseTo(329.882352);
      expect(actual.s).toBeCloseTo(1.0);
      expect(actual.l).toBeCloseTo(0.5);
    });
  });

  describe('toHSV', () => {
    it('should convert a color to HSV', () => {
      // Act
      const color = Color.fromHexString('#ff0080');
      const actual = color.toHSV();

      // Assert
      expect(actual).toEqual({
        h: expect.any(Number),
        s: expect.any(Number),
        v: expect.any(Number),
      });
      expect(actual.h).toBeCloseTo(329.882352);
      expect(actual.s).toBeCloseTo(1.0);
      expect(actual.v).toBeCloseTo(1.0);
    });
  });

  describe('toXYZ', () => {
    it('should convert a color to XYZ', () => {
      // Act
      const color = Color.fromHexString('#ff0080');
      const actual = color.toXYZ();

      // Assert
      expect(actual).toEqual({
        x: expect.any(Number),
        y: expect.any(Number),
        z: expect.any(Number),
      });
      expect(actual.x).toBeCloseTo(0.451349);
      expect(actual.y).toBeCloseTo(0.228222);
      expect(actual.z).toBeCloseTo(0.224513);
    });
  });

  describe('toLab', () => {
    it('should convert a color to Lab', () => {
      // Act
      const color = Color.fromHexString('#ff0080');
      const actual = color.toLab();

      // Assert
      expect(actual).toEqual({
        l: expect.any(Number),
        a: expect.any(Number),
        b: expect.any(Number),
      });
      expect(actual.l).toBeCloseTo(54.888768);
      expect(actual.a).toBeCloseTo(84.532083);
      expect(actual.b).toBeCloseTo(4.065588);
    });
  });

  describe('toLuv', () => {
    it('should convert a color to Luv', () => {
      // Act
      const color = Color.fromHexString('#ff0080');
      const actual = color.toLuv();

      // Assert
      expect(actual).toEqual({
        l: expect.any(Number),
        u: expect.any(Number),
        v: expect.any(Number),
      });
      expect(actual.l).toBeCloseTo(54.888768);
      expect(actual.u).toBeCloseTo(142.07283);
      expect(actual.v).toBeCloseTo(-11.938661);
    });
  });

  describe('toOklab', () => {
    it('should convert a color to Oklab', () => {
      // Act
      const color = Color.fromHexString('#ff0080');
      const actual = color.toOklab();

      // Assert
      expect(actual).toEqual({
        l: expect.any(Number),
        a: expect.any(Number),
        b: expect.any(Number),
      });
      expect(actual.l).toBeCloseTo(0.645347);
      expect(actual.a).toBeCloseTo(0.260077);
      expect(actual.b).toBeCloseTo(0.011148);
    });
  });

  describe('toOklch', () => {
    it('should convert a color to Oklch', () => {
      // Act
      const color = Color.fromHexString('#ff0080');
      const actual = color.toOklch();

      // Assert
      expect(actual).toEqual({
        l: expect.any(Number),
        c: expect.any(Number),
        h: expect.any(Number),
      });
      expect(actual.l).toBeCloseTo(0.645347);
      expect(actual.c).toBeCloseTo(0.260077);
      expect(actual.h).toBeCloseTo(2.454488);
    });
  });

  describe('toLCHab', () => {
    it('should convert a color to LCHab', () => {
      // Act
      const color = Color.fromHexString('#ff0080');
      const actual = color.toLCHab();

      // Assert
      expect(actual).toEqual({
        l: expect.any(Number),
        c: expect.any(Number),
        h: expect.any(Number),
      });
      expect(actual.l).toBeCloseTo(54.888768);
      expect(actual.c).toBeCloseTo(84.629794);
      expect(actual.h).toBeCloseTo(2.75353);
    });
  });

  describe('toLCHuv', () => {
    it('should convert a color to LCHuv', () => {
      // Act
      const color = Color.fromHexString('#ff0080');
      const actual = color.toLCHuv();

      // Assert
      expect(actual).toEqual({
        l: expect.any(Number),
        c: expect.any(Number),
        h: expect.any(Number),
      });
      expect(actual.l).toBeCloseTo(54.888768);
      expect(actual.c).toBeCloseTo(142.573562);
      expect(actual.h).toBeCloseTo(355.196607);
    });
  });

  describe('toAnsi16', () => {
    it('should convert a color to ANSI 16', () => {
      // Act
      const color = Color.fromHexString('#ff0080');
      const actual = color.toAnsi16();

      // Assert
      expect(actual).toEqual({ code: 95 });
    });
  });

  describe('toAnsi256', () => {
    it('should convert a color to ANSI 256', () => {
      // Act
      const color = Color.fromHexString('#ff0080');
      const actual = color.toAnsi256();

      // Assert
      expect(actual).toEqual({ code: 198 });
    });
  });

  describe('toInt', () => {
    it('should convert a color to an integer', () => {
      // Act
      const color = Color.fromHexString('#ff0080');
      const actual = color.toInt();

      // Assert
      expect(actual).toEqual(0xff0080);
    });
  });

  describe('toHexString', () => {
    it('should convert a color to a hex string', () => {
      // Act
      const color = Color.fromInt(0xff0080);
      const actual = color.toHexString();

      // Assert
      expect(actual).toEqual('#FF0080');
    });
  });

  describe('fromInt', () => {
    it.each([
      { input: 0x000000, expected: '#000000' },
      { input: 0xffffff, expected: '#FFFFFF' },
      { input: 0xff0000, expected: '#FF0000' },
      { input: 0x00ff00, expected: '#00FF00' },
      { input: 0x0000ff, expected: '#0000FF' },
    ])("should create a color from an integer '$input'", ({ input, expected }) => {
      // Act
      const actual = Color.fromInt(input);

      // Assert
      expect(actual.toHexString()).toEqual(expected);
    });
  });

  describe('fromString', () => {
    it.each([
      { input: '#000', expected: '#000000' },
      { input: '#fff', expected: '#FFFFFF' },
      { input: '#f00', expected: '#FF0000' },
      { input: '#0f0', expected: '#00FF00' },
      { input: '#00f', expected: '#0000FF' },
      { input: '#000f', expected: '#000000' },
      { input: '#ffff', expected: '#FFFFFF' },
      { input: '#f00f', expected: '#FF0000' },
      { input: '#0f0f', expected: '#00FF00' },
      { input: '#00ff', expected: '#0000FF' },
      { input: '#000000', expected: '#000000' },
      { input: '#ffffff', expected: '#FFFFFF' },
      { input: '#ff0000', expected: '#FF0000' },
      { input: '#00ff00', expected: '#00FF00' },
      { input: '#0000ff', expected: '#0000FF' },
      { input: '#000000ff', expected: '#000000' },
      { input: '#ffffffff', expected: '#FFFFFF' },
      { input: '#ff0000ff', expected: '#FF0000' },
      { input: '#00ff00ff', expected: '#00FF00' },
      { input: '#0000ffff', expected: '#0000FF' },
    ])("should create a color from a string '$input'", ({ input, expected }) => {
      // Act
      const actual = Color.fromHexString(input);

      // Assert
      expect(actual.toHexString()).toEqual(expected);
    });
  });
});
