//
//  BackgroundColorView.swift
//
//  Created by Matthew Westrik on 2020-04-02
//  Copyright © 2020 Matthew Westrik. All rights reserved.
//

import Cocoa

class BackgroundColorView: NSView {

    @IBInspectable var namedBackgroundColor: String = "" {
        didSet {
            updateLayer()
        }
    }

    @IBInspectable var backgroundColor: NSColor = .white {
        didSet {
            updateLayer()
        }
    }
    
    override func updateLayer() {
        if let color = NSColor(named: NSColor.Name(namedBackgroundColor)) {
            layer?.backgroundColor = color.cgColor
        } else {
            layer?.backgroundColor = backgroundColor.cgColor
        }
    }
}
